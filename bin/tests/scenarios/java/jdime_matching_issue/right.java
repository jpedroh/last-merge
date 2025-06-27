package de.fosd.jdime.artifact;

import java.security.MessageDigest;

public abstract class Artifact<T extends Artifact<T>> implements Comparable<T>, StatisticsInterface {
    public boolean hasChanges(Revision revision) {

        if (this.revision.equals(revision)) {
            return false;
        }

        if (!hasMatching(revision)) {
            return true;
        }

        T match = getMatching(revision).getMatchingArtifact(this);

        return getTreeSize() != match.getTreeSize() || !getTreeHash().equals(match.getTreeHash());
    }
}
