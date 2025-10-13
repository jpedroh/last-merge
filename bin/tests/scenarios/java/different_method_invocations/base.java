class Test {
    public void method() {
	jobPlacement.status = Status.Complete;
	plannedPlacement.stepComplete = true;
	Logger.debug("Place {} with {}", part, nozzle.getName());
    }
}
