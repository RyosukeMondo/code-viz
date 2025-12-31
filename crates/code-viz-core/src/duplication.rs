use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tree_sitter::{Node, Parser, Tree};
use crate::models::{CodeLocation, DuplicatePair, DuplicationAnalysis};
use crate::parser::LanguageParser;
use crate::error::{CodeVizError, Result};

pub struct DuplicationDetector {
    min_lines: usize,
    similarity_threshold: f64,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct CodeBlock {
    location: CodeLocation,
    hash: String,
    canonical_text: String,
    line_count: usize,
}

impl DuplicationDetector {
    pub fn new(min_lines: usize, similarity_threshold: f64) -> Self {
        Self {
            min_lines,
            similarity_threshold,
        }
    }

    pub fn run(
        &self,
        files: &[(PathBuf, String)],
        parsers: &HashMap<String, Box<dyn LanguageParser>>,
    ) -> DuplicationAnalysis {
        let all_blocks = self.extract_blocks(files, parsers);
        let duplicate_pairs = self.find_duplicates(&all_blocks);
        let total_duplicated_loc = self.calculate_total_duplicated_loc(&duplicate_pairs);

        DuplicationAnalysis {
            pairs: duplicate_pairs,
            total_duplicated_loc,
        }
    }

    fn extract_blocks(
        &self,
        files: &[(PathBuf, String)],
        parsers: &HashMap<String, Box<dyn LanguageParser>>,
    ) -> Vec<CodeBlock> {
        files
            .iter()
            .flat_map(|(path, content)| {
                let language = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                if let Some(parser) = parsers.get(language) {
                    // Skip files that fail to parse rather than crashing the entire analysis
                    self.parse_file(path, content, parser.as_ref()).unwrap_or_else(|e| {
                        eprintln!("Warning: Failed to parse {:?} for duplication analysis: {}", path, e);
                        vec![]
                    })
                } else {
                    vec![]
                }
            })
            .collect()
    }

    fn parse_file(&self, path: &Path, content: &str, parser: &dyn LanguageParser) -> Result<Vec<CodeBlock>> {
        let mut tree_sitter_parser = Parser::new();
        tree_sitter_parser
            .set_language(parser.get_language())
            .map_err(|e| CodeVizError::parse(
                path.to_path_buf(),
                "unknown",
                None,
                format!("Failed to set tree-sitter language: {:?}", e)
            ))?;

        let tree = tree_sitter_parser.parse(content, None)
            .ok_or_else(|| CodeVizError::parse(
                path.to_path_buf(),
                "unknown",
                None,
                "Failed to parse file - tree-sitter returned None"
            ))?;

        Ok(self.extract_blocks_from_tree(path, content, &tree))
    }

    fn extract_blocks_from_tree(&self, path: &Path, content: &str, tree: &Tree) -> Vec<CodeBlock> {
        let mut blocks = Vec::new();
        let mut queue = vec![tree.root_node()];

        while let Some(node) = queue.pop() {
            if node.kind() == "function_item"
                || node.kind() == "function_declaration"
                || node.kind() == "method_declaration"
            {
                let start_line = node.start_position().row + 1;
                let end_line = node.end_position().row + 1;
                let line_count = end_line - start_line;

                if line_count >= self.min_lines {
                    let canonical_text = self.canonicalize_node(node, content);
                    let hash = format!("{:x}", md5::compute(canonical_text.as_bytes()));
                    let location = CodeLocation {
                        path: path.to_path_buf(),
                        start_line,
                        end_line,
                    };
                    blocks.push(CodeBlock {
                        location,
                        hash,
                        canonical_text,
                        line_count,
                    });
                    continue;
                }
            }

            for i in (0..node.child_count()).rev() {
                if let Some(child) = node.child(i) {
                    queue.push(child);
                }
                // If child is None, skip it - this shouldn't happen but we handle it gracefully
            }
        }
        blocks
    }

    fn canonicalize_node(&self, node: Node, content: &str) -> String {
        let mut canonical_text = String::new();
        let mut queue = vec![node];

        while let Some(current_node) = queue.pop() {
            let kind = current_node.kind();
            if kind == "comment" || kind == "identifier" {
                continue;
            }

            if current_node.child_count() == 0 {
                // It's a leaf node (a token), use its text content.
                // If we can't get valid UTF-8, skip this node
                if let Ok(text) = current_node.utf8_text(content.as_bytes()) {
                    canonical_text.push_str(text);
                    canonical_text.push(' ');
                }
            } else {
                // It's an internal node, use its kind to represent structure.
                canonical_text.push_str(kind);
                canonical_text.push(' ');
            }
            // push children in reverse order to process them in forward order (DFS)
            for i in (0..current_node.child_count()).rev() {
                if let Some(child) = current_node.child(i) {
                    queue.push(child);
                }
            }
        }
        canonical_text
    }

    fn find_duplicates(&self, blocks: &[CodeBlock]) -> Vec<DuplicatePair> {
        let mut pairs = Vec::new();

        // Group blocks by hash
        let mut blocks_by_hash: HashMap<String, Vec<&CodeBlock>> = HashMap::new();
        for block in blocks {
            if !block.hash.is_empty() {
                blocks_by_hash
                    .entry(block.hash.clone())
                    .or_default()
                    .push(block);
            }
        }

        // Find exact duplicates from groups with more than one block
        for (_, duplicated_blocks) in blocks_by_hash.iter().filter(|(_, v)| v.len() > 1) {
            for i in 0..duplicated_blocks.len() {
                for j in i + 1..duplicated_blocks.len() {
                    let block_a = duplicated_blocks[i];
                    let block_b = duplicated_blocks[j];
                    pairs.push(DuplicatePair {
                        original: block_a.location.clone(),
                        duplicate: block_b.location.clone(),
                        similarity: 1.0,
                        line_count: block_a.line_count,
                    });
                }
            }
        }

        // For similarity check, we only need one representative from each hash group.
        let unique_blocks: Vec<&CodeBlock> = blocks_by_hash.values().map(|v| v[0]).collect();

        for i in 0..unique_blocks.len() {
            for j in i + 1..unique_blocks.len() {
                let block_a = unique_blocks[i];
                let block_b = unique_blocks[j];

                let distance =
                    levenshtein::levenshtein(&block_a.canonical_text, &block_b.canonical_text);
                let len_a = block_a.canonical_text.len();
                let len_b = block_b.canonical_text.len();

                if len_a == 0 || len_b == 0 {
                    continue;
                }

                let max_len = std::cmp::max(len_a, len_b) as f64;
                let similarity = 1.0 - (distance as f64 / max_len);

                if similarity >= self.similarity_threshold {
                    // Found a similar pair. Now, create pairs for all blocks in their respective hash groups.
                    // These lookups should always succeed since unique_blocks came from blocks_by_hash.values()
                    if let (Some(blocks_a), Some(blocks_b)) =
                        (blocks_by_hash.get(&block_a.hash), blocks_by_hash.get(&block_b.hash)) {
                        for ba in blocks_a {
                            for bb in blocks_b {
                                pairs.push(DuplicatePair {
                                    original: ba.location.clone(),
                                    duplicate: bb.location.clone(),
                                    similarity,
                                    line_count: ba.line_count,
                                });
                            }
                        }
                    }
                    // If lookups fail (shouldn't happen), skip this pair silently
                }
            }
        }

        pairs
    }

    fn calculate_total_duplicated_loc(&self, pairs: &[DuplicatePair]) -> usize {
        let mut duplicated_lines = HashSet::new();
        for pair in pairs {
            for line in pair.original.start_line..=pair.original.end_line {
                duplicated_lines.insert((pair.original.path.clone(), line));
            }
            for line in pair.duplicate.start_line..=pair.duplicate.end_line {
                duplicated_lines.insert((pair.duplicate.path.clone(), line));
            }
        }
        duplicated_lines.len()
    }
}
