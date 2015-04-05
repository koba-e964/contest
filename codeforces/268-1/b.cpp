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

const int N = 100010;

int n,a,b;
ll p[N];
int aa[N], bb[N];
int ae[N], be[N];
typedef pair<int,int> pi;

int disj[N];

int root(int x) {
	if(disj[x] == x) return x;
	return disj[x] = root(disj[x]);
}

int join(int x,int y) {
	disj[root(x)] = root(y);
}

int main(void){
	cin >> n >> a >> b;
	set<int> ps;
	map<int,int> pis;
	REP(i,0,n) {
		cin >> p[i];
		ps.insert(p[i]);
		pis.insert(pi(p[i],i));
	}
	REP(i,0,n) {
		disj[i] = i;
	}
	REP(i,0,n) {
		aa[i] = ps.count(a-p[i]);
		if(aa[i]) {
			ae[i] = pis[a-p[i]];
			join(i,ae[i]);
		}
		bb[i] = ps.count(b-p[i]);
		if(bb[i]) {
			be[i] = pis[b-p[i]];
			join(i,be[i]);
		}
	}
	REP(i,0,n) {
		aa[root(i)] &= aa[i];
		bb[root(i)] &= bb[i];
	}
	REP(i,0,n) {
		if(aa[i] || bb[i]) {
		} else {
			cout << "NO" << endl;
			return 0;
		}
	}
	cout << "YES" << endl;
	REP(i,0,n) {
		if(aa[root(i)]) cout << 0;
		else cout << 1;
		cout << (i == n-1 ? "\n": " ");
	}
}
