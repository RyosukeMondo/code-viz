"""Sample Python file for testing"""
from typing import List, Optional, Dict


class DataProcessor:
    """A simple data processing class"""

    def __init__(self):
        self.data: List[int] = []
        self.cache: Dict[str, int] = {}

    def add_data(self, value: int) -> None:
        """Add a value to the data list"""
        self.data.append(value)

    def get_data(self) -> List[int]:
        """Get all data"""
        return self.data.copy()

    def calculate_sum(self) -> int:
        """Calculate sum of all data"""
        cache_key = "sum"
        if cache_key in self.cache:
            return self.cache[cache_key]

        total = sum(self.data)
        self.cache[cache_key] = total
        return total

    def calculate_average(self) -> Optional[float]:
        """Calculate average of all data"""
        if not self.data:
            return None

        return sum(self.data) / len(self.data)

    def filter_positive(self) -> List[int]:
        """Filter positive numbers"""
        return [x for x in self.data if x > 0]

    def clear(self) -> None:
        """Clear all data and cache"""
        self.data.clear()
        self.cache.clear()


def process_batch(values: List[int]) -> Dict[str, float]:
    """Process a batch of values"""
    processor = DataProcessor()

    for value in values:
        processor.add_data(value)

    return {
        "sum": processor.calculate_sum(),
        "average": processor.calculate_average() or 0.0,
        "positive_count": len(processor.filter_positive()),
    }
