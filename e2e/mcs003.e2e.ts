describe('MCS-003 fixed overview and trends', () => {
	it('keeps the overview fixed while only the process table scrolls', async () => {
		await $('[data-testid="process-row"]').waitForExist({ timeout: 15_000 });
		await expect($('[aria-label^="CPU trend"]')).toBeDisplayed();
		await expect($('[aria-label^="Memory trend"]')).toBeDisplayed();

		const before = await browser.execute(() => {
			const overview = document.querySelector('[data-testid="system-overview"]');
			const processRegion = document.querySelector('[data-testid="process-scroll-region"]');
			if (!(overview instanceof HTMLElement) || !(processRegion instanceof HTMLElement)) {
				return null;
			}

			return {
				overviewTop: overview.getBoundingClientRect().top,
				scrollable: processRegion.scrollHeight > processRegion.clientHeight,
			};
		});
		expect(before).not.toBeNull();
		expect(before?.scrollable).toBe(true);

		await browser.execute(() => {
			const processRegion = document.querySelector('[data-testid="process-scroll-region"]');
			if (processRegion instanceof HTMLElement) {
				processRegion.scrollTop = processRegion.scrollHeight;
			}
		});

		const after = await browser.execute(() => {
			const overview = document.querySelector('[data-testid="system-overview"]');
			const processRegion = document.querySelector('[data-testid="process-scroll-region"]');
			if (!(overview instanceof HTMLElement) || !(processRegion instanceof HTMLElement)) {
				return null;
			}

			return {
				overviewTop: overview.getBoundingClientRect().top,
				scrollTop: processRegion.scrollTop,
			};
		});
		expect(after?.scrollTop).toBeGreaterThan(0);
		expect(after?.overviewTop).toBe(before?.overviewTop);
	});
});
