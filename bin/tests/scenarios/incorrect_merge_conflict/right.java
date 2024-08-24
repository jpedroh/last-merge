public class Main {
    public static void main() {
         return retrieve().with("since",since)
            .asPagedIterable(
                "/repositories",
                GHRepository[].class,
                item -> item.wrap(GitHub.this) );
    }
}
