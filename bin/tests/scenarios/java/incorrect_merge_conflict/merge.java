public class Main {
    public static void main() {
        return
<<<<<<<
new PagedIterable<GHRepository>() {
            @Override
            public PagedIterator<GHRepository> _iterator(int pageSize) {
                return new PagedIterator<GHRepository>(retrieve().with("since",since).asIterator("/repositories", GHRepository[].class, pageSize)) {
                    @Override
                    protected void wrapUp(GHRepository[] page) {
                        for (GHRepository c : page)
                            c.wrap(GitHub.this);
                    }
                };
            }
        }
=======
retrieve().with("since",since)
            .asPagedIterable(
                "/repositories",
                GHRepository[].class,
                item -> item.wrap(GitHub.this) )
>>>>>>>
;
    }
}