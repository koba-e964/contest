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

const int N = 105;
const int K = 240;
const int DEBUG = 0;

int n,k;

int p[105];

double dp[N][K][K];



double solve(int v, int a, int b) {
	if(DEBUG) {
		printf("v =%d, a=%d, b=%d\n", v, a, b);
	}
	if (dp[v][a][b] >= 0) {
		return dp[v][a][b];
	}
	if (v >= n) {
		return k - a - b;
	}
	double sum = 0.0;
	if (k >= a + 2 * b + 2 || (a == 0 && b == 0)) {
		if (a == 0 && b == 0) {
			sum += solve(v+1, 1, 0) * (100-p[v]) / 100.0;
		} else {
			sum += solve(v+1, a, b + 1) * (100-p[v]) / 100.0;
		}
	} else {
		sum += solve(v+1, a, b) * (100-p[v]) / 100.0;
	}
	if (k >= a + b + 1) {
		if (b == 0) {
			sum += solve(v+1, a+1, 0) * p[v] / 100.0;
			if(DEBUG) cout << sum << "$$$$" << endl;
		} else {
			sum += solve(v+1, a + 2, b - 1) * p[v] / 100.0;
		}
	} else {
		sum += solve(v+1, a, b) * p[v] / 100.0;
	}
	dp[v][a][b] = sum;
	if(DEBUG) {
		printf("v =%d, a=%d, b=%d, sum = %.7f\n", v, a, b, sum);
	}
	return sum;
}



int main(void){
	cin >> n >> k;
	REP(i,0,n) {
		cin>>p[i];
	}
	REP(i,0,n+1) {
		REP(j,0,K) {
			REP(q,0,K) {
				dp[i][j][q] = -1.0;
			}
		}
	}
	printf("%.10f\n", solve(0,0,0));
	
}
