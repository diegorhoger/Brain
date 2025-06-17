"""
Type stubs for the brain module
Rust-Python bindings for the Brain AI engine
"""

from typing import Dict, List, Optional, Any


class PySegment:
    """Represents a text segment with classification and confidence."""
    text: str
    confidence: float
    segment_type: str


class PyMemoryResult:
    """Represents a memory query result."""
    content: str
    relevance: float
    memory_type: str


class PySimulationResult:
    """Represents the result of a simulation."""
    steps: int
    outcome: str
    confidence: float
    metadata: Optional[Dict[str, str]]


class BrainEngine:
    """Main Brain AI engine interface."""
    
    def __init__(self) -> None:
        """Initialize a new Brain engine instance."""
        ...
    
    def segment(self, text: str) -> List[PySegment]:
        """Segment text into classified parts."""
        ...
    
    def learn(self, knowledge: str, priority: str) -> bool:
        """Store knowledge with specified priority."""
        ...
    
    def simulate(self, scenario: str, max_steps: int = 5, 
                 confidence_threshold: float = 0.1) -> PySimulationResult:
        """Run a simulation of the given scenario."""
        ...
    
    def query_memory(self, query: str, 
                     limit: int = 10) -> List[PyMemoryResult]:
        """Query stored memories for relevant information."""
        ...
    
    def get_status(self) -> Dict[str, str]:
        """Get current engine status and statistics."""
        ...
    
    def get_config(self) -> Dict[str, Any]:
        """Get current configuration settings."""
        ...
    
    def update_config(self, settings: Dict[str, str]) -> bool:
        """Update configuration with new settings."""
        ...


def segment_text(text: str) -> List[PySegment]:
    """Convenience function for text segmentation."""
    ...


def quick_query(query: str) -> bool:
    """Convenience function for quick queries."""
    ...


__all__ = [
    'BrainEngine',
    'PySegment', 
    'PyMemoryResult',
    'PySimulationResult',
    'segment_text',
    'quick_query'
] 