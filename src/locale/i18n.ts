import { derived, writable } from 'svelte/store';
export const dict = writable();
export const locale = writable('en');

const localizedDict = derived([dict, locale], ([$dict, $locale]) => {
	if (!$dict || !$locale) return;
	return $dict[$locale];
});

const getMessageFromLocalizedDict = (id: string, localizedDict: any) => {
	const splitId = id.split('.');
	let message = { ...localizedDict };
	splitId.forEach((partialId: string | number) => {
		message = message[partialId];
	});

	return message;
};

const createMessageFormatter = (localizedDict: any) => (id: any) =>
	getMessageFromLocalizedDict(id, localizedDict);

export const t = derived(localizedDict, ($localizedDict) => {
	return createMessageFormatter($localizedDict);
});
