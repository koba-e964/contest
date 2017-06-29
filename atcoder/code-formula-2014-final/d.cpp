/*
 * Reference: http://kmjp.hatenablog.jp/entry/2014/10/08/0900
 */
#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int DEBUG = 0;

struct mov {
	int m, s, e;
};

istream &operator>>(istream &os, mov &m) {
	os >> m.m >> m.s >> m.e;
	return os;
}

bool mov_cmp(const mov &m1, const mov &m2) {
	return m1.s < m2.s || (m1.s == m2.s && (m1.e < m2.e || (m1.e == m2.e && m1.m < m2.m)));
}

int solve(const vector<int> &h, const vector<mov> &ms) {
	int n = h.size();
	vector<map<int, int> > dp(n);
	vector<int> dpm(n, -1);
	REP(i, 0, n) {
		dp[i] = map<int, int>();
	}
	REP(i, 0, n) {
		vector<bool> vis(n + 1, false);
		dp[i][1] = h[0];
		for (int k = i - 1; k >= 0; --k) {
			if (vis[ms[k].m]) { continue; }
			if (ms[k].e > ms[i].s) { continue; }
			vis[ms[k].m] = true;
			if (ms[k].m != ms[i].m) {
				if (dpm[k] >= 0) {
					dp[i][1] = max(dp[i][1], dpm[k] + h[0]);
				}
			} else {
				for (map<int, int>::iterator it = dp[k].begin();
						 it != dp[k].end(); ++it) {
					dp[i][it->first + 1] =
						max(dp[i][it->first + 1], it->second + h[it->first]);
				}
			}
		}
		for (map<int, int>::iterator it = dp[i].begin();
				 it != dp[i].end(); ++it) {
			dpm[i] = max(dpm[i], it->second);
		}
		assert (dpm[i] >= 0);
	}
	int ma = 0;
	REP(i, 0, n) {
		ma = max(ma, dpm[i]);
	}
	return ma;
}

int main(void) {
	int n;
	cin >> n;
	vector<int> h(n);
	vector<mov> ms(n);
	REP(i, 0, n) {
		cin >> h[i];
	}
	REP(i, 0, n) {
		cin >> ms[i];
	}
	sort(ms.begin(), ms.end(), mov_cmp);
	cout << solve(h, ms) << endl;
}
