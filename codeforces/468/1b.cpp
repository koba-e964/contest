#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 5001;
int dp[N][26][26];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  int n = s.length();
  REP(i, 0, n) {
    int ch = s[i] - 'a';
    REP(j, 0, n) {
      int dh = s[(i + j) % n] - 'a';
      dp[j][ch][dh] += 1;
    }
  }
  int cnt = 0;
  REP(j, 0, 26) {
    int ma = 0;
    REP(i, 1, n) {
      int uq = 0;
      REP(k, 0, 26) {
	if (dp[i][j][k] == 1) {
	  uq++;
	}
      }
      ma = max(ma, uq);
    }
    cnt += ma;
  }
  cout << setprecision(15) << ((double) cnt / (double) n) << "\n";
}
