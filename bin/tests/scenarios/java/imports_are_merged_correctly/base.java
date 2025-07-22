package de.fosd.jdime.merge;

import java.util.List;
import java.util.Objects;
import java.util.logging.Level;
import java.util.logging.Logger;

import AST.*;
import de.fosd.jdime.operations.ConflictOperation;
import de.fosd.jdime.operations.DeleteOperation;
import de.fosd.jdime.operations.MergeOperation;

import static de.fosd.jdime.artifact.Artifacts.root;
import static de.fosd.jdime.strdump.DumpMode.PLAINTEXT_TREE;
