#include <algorithm>
#include <cassert>
#include <cctype>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;
void add(ll &x, ll y) {
  x = (x + y) % mod;
}


const int A=300;

ll dp[27][A];

const int N = 1000;
ll comb[N][N];
void init(void) {
  comb[0][0] = 1;
  REP(i, 1, N) {
    REP(j, 0, N) {
      add(comb[i][j], comb[i - 1][j]);
      if (j > 0) {
	add(comb[i][j], comb[i - 1][j - 1]);
      }
    }
  }
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  VI freq(26);
  REP(i, 0, 26) {
    cin >> freq[i];
  }
  dp[0][0] = 1;
  int len = 0;
  REP(i, 0, 26) {
    int nlen = len + freq[i];
    if (freq[i] == 0) {
      REP(j, 0, A) {
        dp[i + 1][j] = dp[i][j];
      }
      continue;
    }
    REP(j, 1, freq[i] + 1) {
      // j: the number of concatenated components
      int numint = freq[i] - j; // the number of holes  
      REP(k, 0, len + 2) {
	REP(l, 0, k + 1) { // the number of holes in the resulting string - numint
	  int nxt = l + numint;
	  if (nxt < 0 || nxt >= A) continue;
	  ll tmp = comb[k][k - l];
	  if (j - (k - l) < 0) continue;
	  tmp = tmp * comb[len + 1 - k][j - (k - l)] % mod;
	  tmp = tmp * comb[freq[i]-1][j-1] % mod;
	  add(dp[i + 1][nxt], dp[i][k] * tmp);
	}
      }
    }
    len = nlen;
  }
  cout << dp[26][0] << endl;
}
