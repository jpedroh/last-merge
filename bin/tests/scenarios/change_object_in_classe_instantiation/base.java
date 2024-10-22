public class GitHubBuilder {
    public GitHubBuilder withProxy(final Proxy p) {
        return withConnector(new HttpConnector() {
            public HttpURLConnection connect(URL url) throws IOException {
                return (HttpURLConnection) url.openConnection(p);
            }
        });
    }
}