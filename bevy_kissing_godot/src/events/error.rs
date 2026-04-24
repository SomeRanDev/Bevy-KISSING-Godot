use crate::entity_or_node_id::EntityOrNodeId;

#[derive(Debug, thiserror::Error)]
pub(super) enum Error {
	#[error("cannot add child to itself")]
	CannotAddChildToItself,

	#[error("cannot remove child from itself")]
	CannotRemoveChildFromItself,

	#[error("node doesn't exist for entity {0}")]
	NodeDoesntExist(EntityOrNodeId),

	#[error("parent node doesn't exist for entity {0}")]
	ParentNodeDoesntExist(EntityOrNodeId),

	#[error("child node doesn't exist for entity {0}")]
	ChildNodeDoesntExist(EntityOrNodeId),

	#[error("node doesn't match desired type {0}")]
	NodeWrongType(EntityOrNodeId),
}
