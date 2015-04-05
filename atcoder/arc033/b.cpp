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



int main(void){
	int a, b;
	set<int> sa;
	cin >> a >> b;
	REP(i, 0, a) {
		int t;
		cin >> t;
		sa.insert(t);
	}
	int c = 0;
	REP(i, 0, b) {
		int t;
		cin >> t;
		if (sa.count(t)) {
			c ++;
		}
	}
	printf("%.10f\n", (double)c / (a + b - c));
}
