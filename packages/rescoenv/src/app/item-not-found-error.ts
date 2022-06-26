export class ItemNotFoundError extends Error {
	constructor(workitemId: number) {
		super(`Item ${workitemId} not found.`);
	}
}
