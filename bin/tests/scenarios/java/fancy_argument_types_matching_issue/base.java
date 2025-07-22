package de.fosd.jdime.common;

import AST.*;

public class ASTNodeArtifact extends Artifact<ASTNodeArtifact> {
	private ASTNodeArtifact(final ASTNode<?> astnode) {
		assert (astnode != null);
		this.astnode = astnode;

		this.initializeChildren();
	}

	public ASTNodeArtifact(final FileArtifact artifact) {
		assert (artifact != null);

		setRevision(artifact.getRevision());

		ASTNode<?> astnode;
		if (artifact.isEmpty()) {
			astnode = new ASTNode<>();
		} else {
			Program p = initProgram();
			p.addSourceFile(artifact.getPath());
			astnode = p;
		}

		this.astnode = astnode;
		this.initializeChildren();
		renumberTree();
	}
}
