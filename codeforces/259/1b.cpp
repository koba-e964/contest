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

const bool DEBUG = 0;

int gcd(int x,int y) {
	if(y)return gcd(y, x%y);
	return x;
}

int p[16]= {2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53};

// v<=58
int factset(int v) {
	int set = 0;
	int  i=0;
	while(v > 1 && i <16) {
		if(v%p[i]) {
			i++;
			continue;
		}
		set |= 1 <<i;
		v /= p[i];
	}
	if (v > 1 && i == 16) assert(0);
	return set;
}


const int INF = 100000000;
const int B=1<<16;
int n;
int a[100];

int dp[110][B]; 

vector<int> trace(int i, int j) {
	if(i == 0) {
		int t = dp[0][j];
		int v;
		if (factset(a[0] + t) == j) {
			v = a[0] + t;
		} else if (factset(a[0]-t) == j) {
			v = a[0]-t;
		} else {
			assert(0);
		}
		vector<int> r;
		r.push_back(v);
		return r;
	}
	REP(k,1,59) {
		int s = factset(k);
		if((s & j) != s) { continue; }
		if (dp[i][j] - dp[i-1][s^j] == abs(k - a[i])) {
			vector<int> sub = trace(i-1,s^j);
			sub.push_back(k);
			return sub;
		}
	}
	assert(0);
}
int main(void){
	cin>>n;
	REP(i,0,n) {
		cin >> a[i];
	}
	REP(k,0,B) {
		dp[0][k] = INF;
	}
	REP(i,1,59) {
		int s = factset(i);
		dp[0][s] = min(dp[0][s],abs(i-a[0]));
	}
	REP(i,1,n) {
		REP(k,0,B) {
			dp[i][k] = INF;
		}
		REP(j,1,59) {
			// dp[i][?] = dp[i-1][?] + abs(j - a[i])
			int s = factset(j);
			REP(k,0,B) {
				if(s & k) { continue; }
				int &m = dp[i][s | k];
				m = min(m, dp[i-1][k] + abs(j - a[i]));
			}
		}
	}
	int mini = 0;
	REP(i,1,B) {
		if (dp[n-1][mini] > dp[n-1][i]) {
			mini = i;
		}
	}
	if(DEBUG) {
		REP(j,0,n) {
			int mi = 0;
			REP(i,1,B) {
				if (dp[j][mi] > dp[j][i]) {
					mi = i;
				}
			}
			cout << "dp[" << j << "][" << mi << "] = " << dp[j][mi] <<endl;
		}
	}
	if(DEBUG) {
		cout << "mini = " <<mini << endl;
	}
	vector<int> res = trace(n-1,mini);
	REP(i,0,n) {
		cout << res[i] << (i == n-1 ? "\n" : " ");
	}
}
