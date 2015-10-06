#include <cassert>
#include <cmath>
#include <cstdlib>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;


vector<pair<int, int> > gacha(void) {
	const int W = 1500;
	const int N = 100;
	const int TH = 100;
	vector<pair<int, int> > res(N);
	for (int i = N; i >= 1; --i) {
		bool found = 0;
		REP(t, 0, TH) {
			int x = rand() % (W - 2 * i + 1) + i;
			int y = rand() % (W - 2 * i + 1) + i;
			bool ok = 1;
			REP(j, i, N) {
				int dist = pow(x - res[j].first, 2) + pow(y - res[j].second, 2);
				if (dist < pow(i + j + 1, 2)) { ok = 0; break; }
			}
			if (ok) {
				found = 1;
				res[i - 1].first = x;
				res[i - 1].second = y;
				break;
			}
		}
		if (! found) {
			return vector<pair<int, int> >();
		}
	}
	return res;
}


int main(void){
	srand(0xdeadc0de);
	REP(loop_cnt, 0, 1000) {
		vector<pair<int, int> > res = gacha();
		if (res.size() > 0) {
			REP(i, 0, 100) {
				cout << res[i].first << " " << res[i].second << endl;
			}
			return 0;
		}
	}
	assert (!"__unreachable__");
}
