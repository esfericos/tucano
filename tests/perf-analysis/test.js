// k6 run ./test.js

import http from 'k6/http';
import { check, sleep } from 'k6';
import { Trend } from 'k6/metrics';

const API_URL = 'http://localhost:8080/factors';

const easyResponseTimes = new Trend('easy_response_times', true);
const mediumResponseTimes = new Trend('medium_response_times', true);
const hardResponseTimes = new Trend('hard_response_times', true);

export const options = {
  scenarios: {
    easyNumbers: {
      executor: 'constant-vus',
      vus: 8,
      duration: '30s',
      exec: 'easyNumbers',
    },
    mediumNumbers: {
      executor: 'constant-vus',
      vus: 8,
      duration: '30s',
      exec: 'mediumNumbers',
      startTime: '30s',
    },
    hardNumbers: {
      executor: 'constant-vus',
      vus: 8,
      duration: '30s',
      exec: 'hardNumbers',
      startTime: '60s',
    },
  },
};

export function easyNumbers() {
  const numbers = [
    '12345', '67890', '111213', '141516', '171819', 
    '202122', '232425', '262728', '293031', '323334'
  ];
  testAPI(numbers, 'easy');
}

export function mediumNumbers() {
  const numbers = [
    '12345678901234567890', '98765432109876543210', '11223344556677889900',
    '99887766554433221100', '10203040506070809000', '50607080901020304050',
    '21436587092143658709', '13579246801357924680', '98765412309876541230',
    '19283746501928374650'
  ];
  testAPI(numbers, 'medium');
}

export function hardNumbers() {
  const numbers = [
    '123456789012345677777', '987654321098765432777', '112233445566778899075',
    '998877665544332211057', '102030405060708090057', '506070809010203040575',
    '214365870921436587577', '135792468013579246875', '9876541230987654123775',
    '19283746501928374655705'
  ];
  testAPI(numbers, 'hard');
}

function testAPI(numbers, tag) {
  const trends = {
    easy: easyResponseTimes,
    medium: mediumResponseTimes,
    hard: hardResponseTimes,
  };

  for (const number of numbers) {
    const res = http.get(`${API_URL}?number=${number}`, {
      tags: { scenario: tag },
    });

    check(res, {
      'status is 200': (r) => r.status === 200,
    });

    trends[tag].add(res.timings.duration);

    sleep(1); 
  }
}
