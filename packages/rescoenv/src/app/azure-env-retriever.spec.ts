import * as azdev from 'azure-devops-node-api';
import { IWorkItemTrackingApi } from 'azure-devops-node-api/WorkItemTrackingApi';
import { AzureEnvRetriever } from './azure-env-retriever';
import { ItemNotFoundError } from './item-not-found-error';

const WORKITEM_ENVS: { [key: number]: string | undefined } = {
	1234: 'Some env for 1234',
	9999: '9999 env',
	11: undefined,
};

describe('AzureEnvRetriever', () => {
	let retriever: AzureEnvRetriever;
	let mockGetWorkItem: jest.SpyInstance;

	beforeEach(() => {
		retriever = new AzureEnvRetriever('', '');

		mockGetWorkItem = jest.fn();
		const mockWiApi: IWorkItemTrackingApi = {
			getWorkItem: mockGetWorkItem,
		} as unknown as IWorkItemTrackingApi;

		jest.spyOn(azdev.WebApi.prototype, 'getWorkItemTrackingApi').mockResolvedValue(mockWiApi);

		mockGetWorkItem.mockImplementation(async (workitemId: number) => {
			if (!Object.keys(WORKITEM_ENVS).includes(workitemId + '')) {
				return undefined;
			}

			const envValue = WORKITEM_ENVS[workitemId];

			return {
				id: workitemId,
				rev: 2,
				fields: {
					'Custom.EnvironmenttoReproduce': envValue,
				},
			};
		});
	});

	describe('retrieveFor', () => {
		it('returns contents of Environment field of the workitem', async () => {
			expect(await retriever.retrieveFor(1234)).toBe('Some env for 1234');
		});

		it('returns contents of Environment field of the another workitem', async () => {
			expect(await retriever.retrieveFor(9999)).toBe('9999 env');
		});

		it('throws ItemNotFoundError if the workitem id does not exist', async () => {
			await expect(retriever.retrieveFor(1)).rejects.toEqual(new ItemNotFoundError(1));
		});

		it('throws original error if the retrieval fails', async () => {
			mockGetWorkItem.mockRejectedValue(new Error('Some error'));
			await expect(retriever.retrieveFor(1)).rejects.toEqual(new Error('Some error'));
		});

		it('returns empty string if the workitem has no fields', async () => {
			mockGetWorkItem.mockResolvedValue({});
			expect(await retriever.retrieveFor(1)).toBe('');
		});

		it('returns empty string if the workitem has the environment field undefined', async () => {
			expect(await retriever.retrieveFor(11)).toBe('');
		});
	});
});
