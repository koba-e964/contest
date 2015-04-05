#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N=200100;
int n;
int b[N];
ll cnt(int v) {
	int c=0;
	REP(i,0,n) {
		if(b[i] == v) c++;
	}
	return c;
}
int main(void){
	cin>>n;
	REP(i,0,n) cin >> b[i];
	int ma=b[0], mi=b[0];
	REP(i,0,n) {
		ma = max(ma,b[i]);
		mi = min(mi,b[i]);
	}
	if (ma != mi) {
		cout << (ma-mi) << " " << cnt(ma) * cnt(mi) << endl;
	} else {
		ll c = cnt(ma);
		cout << 0 << " " << c * (c-1) / 2 << endl;
	}
}
