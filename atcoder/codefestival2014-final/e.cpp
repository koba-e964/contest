#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;



int main(void){
	int n;
	cin >> n;
	VI r(n);
	REP(i, 0, n) {
		cin >> r[i];
	}
	VI dpu(n), dpd(n); // dpu[i] := max{n | x_i_1 ... >x_i_{n-1} < x_i_n and i_n = i}
	dpu[0] = dpd[0] = 1;
	const int minf = -100000;
	REP(i, 1, n) {
		int mu = minf;
		int md = minf;
		REP(j, 0, i) {
			if (r[j] > r[i]) {
				md = max(md, dpu[j] + 1);
			}
			if (r[j] < r[i]) {
				mu = max(mu, dpd[j] + 1);
			}
		}
		if (md >= 0) { dpd[i] = md; }
		if (mu >= 0) { dpu[i] = mu; }
	}
	int m = minf;
	REP(i, 0, n) {
		m = max(m, max(dpd[i], dpu[i]));
	}
	if (m >= 3) {
		cout << m << endl;
	} else {
		cout << 0 << endl;
	}
}
