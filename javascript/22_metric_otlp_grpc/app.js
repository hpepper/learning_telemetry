'use strict';

const { metrics } = require('@opentelemetry/api');

const meter = metrics.getMeter('mylibraryname');

const counter = meter.createCounter('connections', {
  description: 'Total connections over time',
});

counter.add(1, { example_key: 'example_value' });

setInterval(() => {
  const randomCount = Math.floor(Math.random() * 10);
  counter.add(randomCount, { example_key: 'example_value' });
}, 1000);
