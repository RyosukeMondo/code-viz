/**
 * Heat Haze Shader Module
 * Provides shader materials for high-complexity building visualization
 * @module components/visualizations/3d/shaders/HeatHazeShader
 */

import * as THREE from 'three';
import { COMPLEXITY_THRESHOLDS, PERFORMANCE_THRESHOLDS, ERROR_MESSAGES } from '../utils/constants';

/**
 * Vertex shader for heat haze effect
 * Passes UV coordinates, normals, and position to fragment shader
 */
const VERTEX_SHADER = `
precision mediump float;

varying vec2 vUv;
varying vec3 vNormal;
varying vec3 vPosition;

void main() {
  // Pass UV coordinates to fragment shader
  vUv = uv;

  // Pass normal in world space
  vNormal = normalize(normalMatrix * normal);

  // Pass position in world space
  vPosition = (modelMatrix * vec4(position, 1.0)).xyz;

  // Standard vertex transformation
  gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`;

/**
 * Fragment shader for heat haze effect
 * Creates a distortion effect for high-complexity buildings
 * Used to visually highlight problematic code areas
 */
const FRAGMENT_SHADER = `
precision mediump float;

uniform float time;
uniform float intensity;
uniform vec3 color;

varying vec2 vUv;
varying vec3 vNormal;
varying vec3 vPosition;

/**
 * Simple noise function for UV distortion
 * Based on sine waves to create heat haze effect
 */
float noise(vec2 p) {
  return sin(p.x * 10.0 + time * 2.0) * sin(p.y * 10.0 + time * 1.5) * 0.5 + 0.5;
}

void main() {
  // Calculate UV distortion based on time and intensity
  vec2 distortion = vec2(
    sin(vUv.y * 10.0 + time * 2.0) * intensity * 0.05,
    cos(vUv.x * 10.0 + time * 1.5) * intensity * 0.05
  );

  vec2 distortedUv = vUv + distortion;

  // Add noise-based variation
  float n = noise(vUv);

  // Mix base color with distortion effect
  // Brighter areas indicate more distortion
  vec3 finalColor = color;
  finalColor += vec3(n * intensity * 0.3);

  // Basic lighting using normal
  vec3 lightDir = normalize(vec3(1.0, 1.0, 1.0));
  float diffuse = max(dot(normalize(vNormal), lightDir), 0.0);
  finalColor *= (0.5 + diffuse * 0.5);

  // Add slight glow for high intensity
  float glow = intensity * 0.2;
  finalColor += vec3(glow, glow * 0.5, 0.0); // Orange glow

  gl_FragColor = vec4(finalColor, 1.0);
}
`;

/**
 * Shader uniforms interface
 * Defines the structure of uniform values passed to the shader
 */
export interface HeatHazeUniforms {
  [uniform: string]: { value: number | THREE.Color };
  time: { value: number };
  intensity: { value: number };
  color: { value: THREE.Color };
}

/**
 * Configuration options for HeatHazeShader
 */
export interface HeatHazeShaderOptions {
  complexityThreshold?: number;
  lodDistance?: number;
  enableFallback?: boolean;
}

/**
 * Shader statistics for monitoring
 */
export interface ShaderStats {
  shadersActive: number;
  compilationFailed: boolean;
  complexityThreshold: number;
  lodDistance: number;
  currentTime: number;
}

/**
 * HeatHazeShader class manages shader materials for high-complexity buildings
 * Provides visual effects to highlight problematic code areas
 */
export class HeatHazeShader {
  private complexityThreshold: number;
  private lodDistance: number;
  private enableFallback: boolean;
  private shaderMaterials: Map<string, THREE.ShaderMaterial>;
  private fallbackMaterial: THREE.MeshStandardMaterial | null;
  private compilationFailed: boolean;
  private time: number;

  /**
   * Creates a new HeatHazeShader instance
   * @param options - Configuration options
   */
  constructor(options: HeatHazeShaderOptions = {}) {
    this.complexityThreshold = options.complexityThreshold ?? COMPLEXITY_THRESHOLDS.MEDIUM;
    this.lodDistance = options.lodDistance ?? PERFORMANCE_THRESHOLDS.LOD_DISTANCE;
    this.enableFallback = options.enableFallback ?? true;

    // Track shader materials and their state
    this.shaderMaterials = new Map();
    this.fallbackMaterial = null;
    this.compilationFailed = false;

    // Animation time for shader
    this.time = 0;

    // Try to create shader materials
    this._initializeShaders();
  }

  /**
   * Initializes shader materials with error handling
   */
  private _initializeShaders(): void {
    try {
      // Create fallback material first (in case shader fails)
      this.fallbackMaterial = new THREE.MeshStandardMaterial({
        vertexColors: true,
        roughness: 0.7,
        metalness: 0.2
      });

      console.log('HeatHazeShader: Shader materials initialized successfully');
    } catch (error) {
      console.error('HeatHazeShader: Initialization failed', error);
      console.warn(ERROR_MESSAGES.SHADER_COMPILATION_FAILED);
      this.compilationFailed = true;
    }
  }

  /**
   * Creates a shader material for a specific complexity level
   * @param complexity - Cyclomatic complexity value
   * @param baseColor - Base color for the building
   * @returns Shader or fallback material
   */
  createMaterial(complexity: number, baseColor: THREE.Color): THREE.ShaderMaterial | THREE.MeshStandardMaterial {
    // Return fallback if shaders failed to compile
    if (this.compilationFailed && this.enableFallback) {
      return this.fallbackMaterial!.clone();
    }

    // Don't use shader for low complexity buildings
    if (complexity < this.complexityThreshold) {
      return this.fallbackMaterial!.clone();
    }

    // Calculate intensity based on complexity
    // Scale from 0 to 1 for complexity range [20, 50]
    const intensity = Math.min((complexity - this.complexityThreshold) / 30, 1.0);

    try {
      // Create shader material with uniforms
      const material = new THREE.ShaderMaterial({
        uniforms: {
          time: { value: this.time },
          intensity: { value: intensity },
          color: { value: baseColor.clone() }
        } as HeatHazeUniforms,
        vertexShader: VERTEX_SHADER,
        fragmentShader: FRAGMENT_SHADER,
        side: THREE.FrontSide
      });

      // Store material for later updates
      const key = complexity.toFixed(0);
      this.shaderMaterials.set(key, material);

      return material;
    } catch (error) {
      console.error('HeatHazeShader: Material creation failed', error);
      console.warn(ERROR_MESSAGES.SHADER_COMPILATION_FAILED);
      this.compilationFailed = true;

      // Return fallback
      return this.enableFallback ? this.fallbackMaterial!.clone() : new THREE.MeshStandardMaterial();
    }
  }

  /**
   * Updates shader time uniform for animation
   * Should be called in animation loop
   * @param deltaTime - Time elapsed since last frame (in seconds)
   */
  update(deltaTime: number): void {
    if (this.compilationFailed) return;

    this.time += deltaTime;

    // Update time uniform for all active shader materials
    for (const material of this.shaderMaterials.values()) {
      if (material.uniforms && material.uniforms.time) {
        material.uniforms.time.value = this.time;
      }
    }
  }

  /**
   * Checks if heat haze should be applied based on camera distance
   * Level of Detail optimization - disable expensive effects when far away
   * @param buildingPosition - Position of the building
   * @param camera - Active camera
   * @returns True if effect should be applied
   */
  shouldApplyEffect(buildingPosition: THREE.Vector3, camera: THREE.Camera): boolean {
    const distance = camera.position.distanceTo(buildingPosition);
    return distance <= this.lodDistance;
  }

  /**
   * Disposes of all shader materials to free memory
   */
  dispose(): void {
    // Dispose all shader materials
    for (const material of this.shaderMaterials.values()) {
      material.dispose();
    }
    this.shaderMaterials.clear();

    // Dispose fallback material
    if (this.fallbackMaterial) {
      this.fallbackMaterial.dispose();
      this.fallbackMaterial = null;
    }

    console.log('HeatHazeShader: Disposed all materials');
  }

  /**
   * Returns statistics about shader usage
   * @returns Statistics object
   */
  getStats(): ShaderStats {
    return {
      shadersActive: this.shaderMaterials.size,
      compilationFailed: this.compilationFailed,
      complexityThreshold: this.complexityThreshold,
      lodDistance: this.lodDistance,
      currentTime: this.time
    };
  }
}

/**
 * Helper function to determine if a complexity level requires heat haze
 * @param complexity - Cyclomatic complexity value
 * @param threshold - Threshold for applying effect
 * @returns True if heat haze should be applied
 */
export function shouldUseHeatHaze(complexity: number, threshold: number = COMPLEXITY_THRESHOLDS.MEDIUM): boolean {
  return complexity >= threshold;
}

/**
 * Calculates heat haze intensity from complexity
 * @param complexity - Cyclomatic complexity value
 * @param threshold - Minimum complexity for effect
 * @returns Intensity value between 0 and 1
 */
export function calculateIntensity(complexity: number, threshold: number = COMPLEXITY_THRESHOLDS.MEDIUM): number {
  if (complexity < threshold) return 0;
  return Math.min((complexity - threshold) / 30, 1.0);
}
