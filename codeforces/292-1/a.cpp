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

int a[10];
int b[10];

int main(void){
	int n;
	cin >> n;
	ll t;
	cin >> t;
	REP(i, 0, n) {
		a[t % 10]++;
		t /= 10;
	}
	REP(i, 2, 10) {
		switch(i) {
		case 4:
			b[2] += 2 * a[i];
			b[3] += a[i];
			break;
		case 6:
			b[3] += a[i];
			b[5] += a[i];
			break;
		case 8:
			b[2] += 3 * a[i];
			b[7] += a[i];
			break;
		case 9:
			b[2] += a[i];
			b[3] += 2 * a[i];
			b[7] += a[i];
			break;
		default:
			b[i] += a[i];
			break;
		}
	}
	for (int i = 7; i >= 2; --i) {
		REP(j, 0, b[i]) {
			cout << i;
		}
	}
	cout << endl;
}
