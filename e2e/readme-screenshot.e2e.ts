describe('README screenshot', () => {
	it('captures the current dashboard', async () => {
		await $('[data-testid="process-row"]').waitForExist({ timeout: 15_000 });
		await browser.execute(() => {
			document.querySelector('.e2e-fixtures')?.remove();
			const alertCenter = document.querySelector('[data-testid="alert-center"]');
			alertCenter?.querySelector('.alert-list')?.remove();
			const alertCount = alertCenter?.querySelector('.alert-count');
			if (alertCount instanceof HTMLElement) {
				alertCount.textContent = '0 recorded';
			}
			if (alertCenter instanceof HTMLElement && !alertCenter.querySelector('.alert-empty')) {
				const emptyState = document.createElement('p');
				emptyState.className = 'alert-empty';
				emptyState.textContent = 'No sustained resource alerts recorded.';
				alertCenter.append(emptyState);
			}
			const processRegion = document.querySelector('[data-testid="process-scroll-region"]');
			if (processRegion instanceof HTMLElement) {
				processRegion.scrollTop = 0;
			}
		});
		await browser.saveScreenshot('./docs/images/macsentinel-dashboard.png');
	});
});
