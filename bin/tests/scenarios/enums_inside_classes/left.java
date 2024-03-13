package de.fosd.jdime.stats;

public final class KeyEnums {
    private KeyEnums() {}

    public enum Type {
        FILE,
        DIRECTORY,
        LINE,
        NODE,
        CLASS,
        METHOD,
        TRY
    }

    public enum Level {
        NONE,
        TOP,
        CLASS,
        METHOD
    }
}
