import { AzureEnvRetriever } from './azure-env-retriever';
import * as azdev from 'azure-devops-node-api';
import { IWorkItemTrackingApi } from 'azure-devops-node-api/WorkItemTrackingApi';

describe('AzureEnvRetriever', () => {
	let retriever: AzureEnvRetriever;

	beforeEach(() => {
		retriever = new AzureEnvRetriever('', '');

		const mockGetWorkItem = jest.fn();
		const mockWiApi: IWorkItemTrackingApi = {
			getWorkItem: mockGetWorkItem,
		} as unknown as IWorkItemTrackingApi;

		jest.spyOn(azdev.WebApi.prototype, 'getWorkItemTrackingApi').mockResolvedValue(mockWiApi);

		mockGetWorkItem.mockImplementation((workitemId: number) => ({
			id: workitemId,
			rev: 2,
			fields: {
				'Custom.EnvironmenttoReproduce': `Some env for ${workitemId}`,
			},
		}));
	});

	describe('retrieveFor', () => {
		it('returns contents of Environment field of the workitem', async () => {
			expect(await retriever.retrieveFor(1234)).toBe('Some env for 1234');
		});

		it('returns contents of Environment field of the another workitem', async () => {
			expect(await retriever.retrieveFor(9999)).toBe('Some env for 9999');
		});
	});
});
