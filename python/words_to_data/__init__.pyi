"""Type stubs for words_to_data"""

from typing import Any, Literal

class USLMElement:
      """A hierarchical element in a USLM document tree"""

      @property
      def data(self) -> dict[str, Any]:
          """Element metadata and content"""
          ...

      @property
      def children(self) -> list[USLMElement]:
          """Child elements in document order"""
          ...

      def find(self, path: str) -> USLMElement | None:
          """Find an element by its structural path.

          Args:
              path: The full structural path of the element

          Returns:
              The matching element, or None if not found
          """
          ...

class TextChange:
    """A single word-level change within a text field"""

    @property
    def value(self) -> str:
        """The text value of this change"""
        ...

    @property
    def old_index(self) -> int | None:
        """Position in the original text (None for insertions)"""
        ...

    @property
    def new_index(self) -> int | None:
        """Position in the new text (None for deletions)"""
        ...

    @property
    def tag(self) -> Literal["insert", "delete", "equal"]:
        """The type of change"""
        ...

class FieldChangeEvent:
    """A change detected in a single text content field"""

    @property
    def field_name(self) -> Literal["heading", "chapeau", "proviso", "content", "continuation"]:
        """Which text content field changed"""
        ...

    @property
    def from_date(self) -> str:
        """The publication date of the original version"""
        ...

    @property
    def to_date(self) -> str:
        """The publication date of the new version"""
        ...

    @property
    def old_value(self) -> str:
        """The complete original text of the field"""
        ...

    @property
    def new_value(self) -> str:
        """The complete new text of the field"""
        ...

    @property
    def changes(self) -> list[TextChange]:
        """Word-level changes showing insertions, deletions, and unchanged portions"""
        ...

class TreeDiff:
    """A hierarchical diff between two versions of a USLM document tree"""

    @property
    def root_path(self) -> str:
        """The structural path of the element being compared"""
        ...

    @property
    def changes(self) -> list[FieldChangeEvent]:
        """Text content field changes for this element"""
        ...

    @property
    def from_element(self) -> dict[str, Any]:
        """Metadata from the original version of this element"""
        ...

    @property
    def to_element(self) -> dict[str, Any]:
        """Metadata from the new version of this element"""
        ...

    @property
    def added(self) -> list[dict[str, Any]]:
        """Child elements that were added in the new version"""
        ...

    @property
    def removed(self) -> list[dict[str, Any]]:
        """Child elements that were removed from the old version"""
        ...

    @property
    def child_diffs(self) -> list[TreeDiff]:
        """Recursive diffs for child elements present in both versions"""
        ...

    def find(self, path: str) -> TreeDiff | None:
        """Find a diff by its structural path.

        Args:
            path: The full structural path of the element

        Returns:
            The matching diff, or None if not found
        """
        ...

def parse_uslm_xml(path: str, date: str) -> USLMElement:
    """Parse a USLM XML file and return as a USLMElement.

    Args:
        path: Path to the USLM XML file
        date: Publication date in YYYY-MM-DD format

    Returns:
        Parsed document as a USLMElement tree
    """
    ...

def compute_diff(old_element: USLMElement, new_element: USLMElement) -> TreeDiff:
    """Compute word-level diff between two USLM documents.

    Args:
        old_element: The original (older) version of the element
        new_element: The new (newer) version of the element

    Returns:
        TreeDiff containing all detected changes

    Raises:
        ValueError: If the two elements don't have the same structural path
    """
    ...

__version__: str
__all__: list[str]