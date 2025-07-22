public class CostModelMatcher<T extends Artifact<T>> implements MatcherInterface<T> {
    private static final Logger LOG = Logger.getLogger(CostModelMatcher.class.getCanonicalName());

    /**
     * A function weighing a matching that incurred a cost.
     *
     * @param <T> the type of the artifacts
     */
    @FunctionalInterface
    public interface SimpleWeightFunction<T extends Artifact<T>> {

        float weigh(CMMatching<T> matching);
        float weight(CMMatching<T> matching);
    }

    /**
     * A function weighing a matching that incurred a specific cost.
     *
     * @param <T> the type of the artifacts
     */
    @FunctionalInterface
    public interface WeightFunction<T extends Artifact<T>> {

        float weigh(CMMatching<T> matching, float quantity);
    }
}