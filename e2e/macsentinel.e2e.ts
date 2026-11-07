describe('MacSentinel integrated journeys', () => {
	it('renders the live overview and ranked process evidence', async () => {
		const overview = await $('[data-testid="system-overview"]');
		await expect(overview).toBeDisplayed();
		await expect($('[data-testid="process-table"]')).toBeDisplayed();
		await expect($('[data-testid="cpu-semantics"]')).toHaveText(expect.stringContaining('one logical core'));
	});

	it('opens process details from the process explorer', async () => {
		await $('[data-testid="process-row"]').waitForExist({ timeout: 10_000 });
		await browser.execute(() => {
			(document.querySelector('[data-testid="process-row"]') as HTMLElement | null)?.click();
		});
		await expect($('[role="dialog"][data-testid="process-detail"]')).toBeDisplayed();
	});

	it('shows sustained CPU alert evidence', async () => {
		await expect($('[data-testid="alert-center"]')).toBeDisplayed();
		const evidence = await $('[data-testid="alert-evidence"]');
		await expect(evidence).toHaveText(expect.stringMatching(/threshold|duration/i));
	});

	it('summarizes monitoring in the menu-bar popover', async () => {
		await browser.execute(() => {
			(document.querySelector('[data-testid="open-popover-fixture"]') as HTMLElement | null)?.click();
		});
		await expect($('[data-testid="menu-bar-summary"]')).toBeDisplayed();
		await expect($$('[data-testid="popover-top-process"]')).toBeElementsArrayOfSize(3);
	});

	it('keeps live process row identities distinct', async () => {
		const rows = await $$('[data-testid="process-row"]');
		expect(rows.length).toBeGreaterThan(1);
		const firstKey = await rows[0].getAttribute('data-process-key');
		const secondKey = await rows[1].getAttribute('data-process-key');
		expect(firstKey).not.toEqual(secondKey);
	});

	it('suppresses repeated alert notifications without deleting evidence', async () => {
		await $('[data-testid="dismiss-alert"]').click();
		await expect($('[data-testid="alert-history-row"]')).toBeDisplayed();
		await expect($('[data-testid="notification-suppressed"]')).toBeDisplayed();
	});

	it('reports memory growth without claiming a cause', async () => {
		await $('[data-testid="load-memory-growth-fixture"]').click();
		const evidence = await $('[data-testid="alert-evidence"]');
		await expect(evidence).toHaveText(expect.stringMatching(/memory|window/i));
		await expect(evidence).not.toHaveText(expect.stringMatching(/caused by/i));
	});

	it('applies settings and deletes retained history after confirmation', async () => {
		await $('[data-testid="open-settings"]').click();
		await $('[data-testid="sampling-interval"]').setValue('5');
		await $('[data-testid="save-settings"]').click();
		await expect($('[data-testid="settings-saved"]')).toBeDisplayed();
		await $('[data-testid="delete-history"]').click();
		await $('[data-testid="confirm-delete-history"]').click();
		await expect($('[data-testid="history-empty"]')).toBeDisplayed();
	});

	it('reports a confirmed termination outcome', async () => {
		await $('[data-testid="open-termination-fixture"]').click();
		await $('[data-testid="terminate-process"]').click();
		await $('[data-testid="confirm-termination"]').click();
		await expect($('[data-testid="termination-outcome"]')).toBeDisplayed();
	});

	it('prevents termination after cancellation or identity change', async () => {
		await $('[data-testid="open-termination-fixture"]').click();
		await $('[data-testid="terminate-process"]').click();
		await $('[data-testid="cancel-termination"]').click();
		await expect($('[data-testid="termination-not-sent"]')).toBeDisplayed();
	});

	it('keeps live monitoring visible when dependencies are denied', async () => {
		await $('[data-testid="load-degraded-fixture"]').click();
		await expect($('[data-testid="system-overview"]')).toBeDisplayed();
		await expect($('[data-testid="storage-degraded"]')).toBeDisplayed();
		await expect($('[data-testid="notifications-denied"]')).toBeDisplayed();
	});

	it('marks a sampling gap and resets alert continuity', async () => {
		await $('[data-testid="load-sampling-gap-fixture"]').click();
		await expect($('[data-testid="sampling-gap"]')).toBeDisplayed();
		await expect($('[data-testid="alert-continuity-reset"]')).toBeDisplayed();
	});
});
