#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

double dp[40][40];

double solve(int a, int b) {
	if(dp[a][b] >= 0) {
		return dp[a][b];
	}
	if(a == 0) return 0;
	double sum = 0;
	sum += (solve(a-1, b-1) + 1) / b;
	sum += (solve(a-1,b-1) + 3) / b * (a-1);
	if(a < b) {
		sum += (solve(a,b-1) + 2) * (b-a) / b;
	}
	dp[a][b] = sum;
	return sum;
}

int main(void){
	string s,k;

	cin>>s;
	cin >> k;
	int t = 36 - k.length();
	int a = 0;
	REP(i,0,s.length()) {
		bool f = true;
		REP(j,0,i) {
			if(s[j] == s[i]) {
				f=false;
			}
		}
		REP(j,0,k.length()) {
			if(k[j] == s[i]) {
				f=false;
			}
		}
		if(f) a++;
	}
	REP(i,0,40) {
		REP(j,0,40) {
			dp[i][j] = -1;
		}
	}
	printf("%.10f\n", solve(a,t) + (s.length() - a));
}
