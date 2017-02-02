#include <bits/stdc++.h>
using namespace std;
const int N = 1010;
#define rep(i, s, n) for (int i = (s); i < (int)(n); ++i)
int dp[N][N];
int main() {
	int n, m;
	cin >> n >> m;
	string s,t;
	cin >> s >> t;
	dp[0][0] = 0;
	rep(i, 0, n + 1) {
		rep(j, i == 0, m + 1) {
			int mi = 1e8;
#define upd(t) mi = min(mi, (t));
			if (i >= 1 && j >= 1 && s[i-1] == t[j-1])
				upd(dp[i-1][j-1])
			if (i >= 1 && j >= 1 && s[i-1] != t[j-1])
				upd(dp[i-1][j-1]+1);
			if(i>=1)
				upd(dp[i-1][j]+1)
			if(j>=1)
				upd(dp[i][j-1]+1)
			dp[i][j]=mi;
		}
		
	}
	cout<<dp[n][m]<<endl;
}
