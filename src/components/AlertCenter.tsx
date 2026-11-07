import type { AlertEvidence } from '../types';

interface AlertCenterProps {
	alerts: AlertEvidence[];
}

export default function AlertCenter({ alerts }: AlertCenterProps) {
	return (
		<section className="alert-center" data-testid="alert-center" aria-labelledby="alert-center-title">
			<div className="section-heading">
				<div>
					<p className="eyebrow">Evidence-based warnings</p>
					<h2 id="alert-center-title">Alerts</h2>
				</div>
				<span className="alert-count">{alerts.length} recorded</span>
			</div>
			{alerts.length === 0 ? (
				<p className="alert-empty">No sustained resource alerts recorded.</p>
			) : (
				<ol className="alert-list">
					{alerts.map((alert) => (
						<li key={alert.id} className="alert-card" data-testid="alert-evidence">
							<span className="alert-icon" aria-hidden="true">!</span>
							<div>
								<strong>{alert.title}</strong>
								<p>{alert.summary}</p>
							</div>
						</li>
					))}
				</ol>
			)}
		</section>
	);
}
