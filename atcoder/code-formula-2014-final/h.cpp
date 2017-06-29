#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
const double EPS=1e-9;

const int N = 50010;
int n,s1,s2;
int a[N],b[N];

VI atob[N], btoa[N];

ll calc(int lim) {
	if (lim <= 0) { return 0; }
	int q = (int)sqrt(lim);
	ll cnt = 0;
	REP(i, 0, n) {
		int alim = min(N - a[i] - 1, q);
		REP(j, 1, alim + 1) {
			// checks x s.t. a[x] == a[i] + j. b[x] == atob[a[i] + j]
			int sup = lim / j + b[i];
			int inf = b[i] + 1;
			VI &target = atob[a[i] + j];
			int res= upper_bound(target.begin(), target.end(), sup) - lower_bound(target.begin(), target.end() , inf);
			cnt += res;
		}
		REP(j, 1, min(N - b[i], lim / q + 1)) {
			int sup = lim / j + a[i];
			int inf = alim + a[i] + 1;
			VI &target = btoa[b[i] + j];
			cnt += upper_bound(target.begin(), target.end(), sup) - lower_bound(target.begin(), target.end() , inf);
		}
	}
	return cnt;
}

int main(void){
	cin >> n >> s1 >> s2;
	REP(i,0,n) {
		cin >> a[i] >> b[i];
		atob[a[i]].push_back(b[i]);
		btoa[b[i]].push_back(a[i]);
	}
	REP(i, 0, N) {
		sort(atob[i].begin(), atob[i].end());
		sort(btoa[i].begin(), btoa[i].end());
	}
	cout << calc(s2) - calc(s1 - 1) << endl;
}
