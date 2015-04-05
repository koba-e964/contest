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
int n;
string f[N], s[N];
int p[N];
int main(void){
	cin >> n;
	REP(i,0,n) {
		cin >> f[i] >> s[i];
		if(s[i] < f[i]) swap(f[i],s[i]);
	}
	REP(i,0,n) {
		cin >> p[i];
		p[i]--;
	}
	string cur = f[p[0]];
	REP(i,1,n) {
		if(cur < f[p[i]]) {
			cur = f[p[i]];
			continue;
		}
		if(cur < s[p[i]]) {
			cur = s[p[i]];
			continue;
		}
		cout << "NO" << endl;
		return 0;
	}
	cout << "YES" << endl;
	return 0;
}
