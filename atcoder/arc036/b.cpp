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

const int N = 300001;

int n;
int a[N];
int asc[N], dsc[N];
int main(void){
	cin >> n;
	REP(i,0,n) {
		cin >> a[i];
	}
	asc[0] = 0;
	REP(i, 1, n) {
		asc[i] = a[i - 1] < a[i] ? asc[i - 1] + 1 : 0;
	}
	dsc[n - 1] = 0;
	for (int i = n - 2; i >= 0; --i) {
		dsc[i] = a[i] > a[i + 1] ? dsc[i + 1] + 1 : 0;
	}
	int m = 0;
	REP(i, 0, n) {
		m = max(m, asc[i] + dsc[i] + 1);
	}
	cout << m << endl;
}
