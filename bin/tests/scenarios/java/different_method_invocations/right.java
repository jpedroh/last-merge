class Test {
    public void method() {
	jobPlacement.status = Status.Complete;
	boardLocation.setPlaced(jobPlacement.placement.getId(), true);
	plannedPlacement.stepComplete = true;
	Logger.debug("Place {} with {}", part, nozzle.getName());
    }
}
