import { getPersonalAccessTokenHandler, WebApi } from 'azure-devops-node-api';
import { ItemNotFoundError } from './item-not-found-error';

/**
 * Class for retrieving the "Environment" field from a work item.
 */
export class AzureEnvRetriever {
	private readonly connection: WebApi;

	constructor(orgUrl: string, token: string) {
		const authHandler = getPersonalAccessTokenHandler(token);
		this.connection = new WebApi(orgUrl, authHandler);
	}

	async retrieveFor(workitemId: number): Promise<string> {
		const wapi = await this.connection.getWorkItemTrackingApi();
		const workitem = await wapi.getWorkItem(workitemId);

		if (!workitem) {
			throw new ItemNotFoundError(workitemId);
		}

		return workitem.fields?.['Custom.EnvironmenttoReproduce'] ?? '';
	}
}
