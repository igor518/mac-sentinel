interface TrendChartProps {
	label: string;
	values: number[];
	color: string;
	formatValue: (value: number) => string;
	maximum?: number;
}

const WIDTH = 240;
const HEIGHT = 54;
const PADDING = 3;

export default function TrendChart({
	label,
	values,
	color,
	formatValue,
	maximum,
}: TrendChartProps) {
	const available = values.filter(Number.isFinite);
	const observedMinimum = available.length > 0 ? Math.min(...available) : 0;
	const observedMaximum = available.length > 0 ? Math.max(...available) : 0;
	const domainMinimum = maximum === undefined ? observedMinimum : 0;
	const domainMaximum = maximum ?? observedMaximum;
	const domainRange = Math.max(domainMaximum - domainMinimum, 1);
	const points = available.map((value, index) => {
		const x = available.length === 1
			? WIDTH / 2
			: PADDING + (index / (available.length - 1)) * (WIDTH - PADDING * 2);
		const y = HEIGHT - PADDING
			- ((value - domainMinimum) / domainRange) * (HEIGHT - PADDING * 2);
		return `${x.toFixed(1)},${y.toFixed(1)}`;
	});
	const singlePointY = available.length === 1
		? maximum === undefined
			? HEIGHT / 2
			: HEIGHT - PADDING
				- ((available[0] - domainMinimum) / domainRange) * (HEIGHT - PADDING * 2)
		: 0;
	const linePoints = available.length === 1
		? `${PADDING},${singlePointY.toFixed(1)} ${WIDTH - PADDING},${singlePointY.toFixed(1)}`
		: points.join(' ');
	const range = available.length === 0
		? 'No available samples'
		: `Range ${formatValue(observedMinimum)} to ${formatValue(observedMaximum)}`;

	return (
		<figure className="trend-chart">
			<svg
				viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
				role="img"
				aria-label={`${label} trend. ${available.length} samples. ${range}.`}
				data-point-count={available.length}
				preserveAspectRatio="none"
			>
				<line className="trend-chart__baseline" x1={PADDING} y1={HEIGHT - PADDING} x2={WIDTH - PADDING} y2={HEIGHT - PADDING} />
				{available.length > 0 && (
					<polyline
						points={linePoints}
						fill="none"
						stroke={color}
						strokeWidth="3"
						strokeLinecap="round"
						strokeLinejoin="round"
						vectorEffect="non-scaling-stroke"
					/>
				)}
			</svg>
			<figcaption>
				<span>Last {available.length} samples</span>
				<span>{range}</span>
			</figcaption>
		</figure>
	);
}
