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


const int N = 400100;
const int B = 20;
int par[N];
ll wei[N];
int cnt;
int anc[B][N];
ll cum[B][N];
int g2par[N];
int g2anc[B][N];
ll g2cum[B][N];
const ll inf = -1e16;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int qq;
  cin >> qq;
  ll last = 0;
  cnt = 1;
  par[0] = -1;
  wei[0] = 0;
  g2par[0] = -1;
  REP(i, 0, B) {
    anc[i][0] = -1;
    cum[i][0] = -inf;
    g2anc[i][0] = -1;
    g2cum[i][0] = 0;
  }
  REP(_, 0, qq) {
    int kind;
    cin >> kind;
    ll p, q;
    cin >> p >> q;
    p ^= last;
    q ^= last;
    p--;
    if (kind == 1) {
      par[cnt] = p;
      wei[cnt] = q;
      anc[0][cnt] = p;
      cum[0][cnt] = wei[p];
      REP(i, 1, B) {
	int w = anc[i - 1][cnt];
        anc[i][cnt] = w == -1 ? -1 : anc[i - 1][w];
	ll ma = cum[i - 1][cnt];
	if (w != -1) {
	  ma = max(ma, cum[i - 1][w]);
	}
	cum[i][cnt] = ma;
      }
      // find the lowest ancestor j that satisfies a[j] >= a[i]
      int cur = cnt;
      for (int i = B - 1; i >= 0; --i) {
	if (cur < 0) break;
	if (cum[i][cur] < q) {
	  cur = anc[i][cur];
	}
      }
      if (cur >= 0) {
	cur = par[cur];
      }
      g2par[cnt] = cur;
      g2anc[0][cnt] = cur;
      g2cum[0][cnt] = q;
      REP(i, 1, B) {
	int w = g2anc[i - 1][cnt];
        g2anc[i][cnt] = w == -1 ? -1 : g2anc[i - 1][w];
	ll sum = g2cum[i - 1][cnt];
	if (w != -1) {
	  sum += g2cum[i - 1][w];
	}
	g2cum[i][cnt] = sum;
      }
      cnt += 1;
      continue;
    }
    int diff = 0;
    int cur = p;
    ll rem = q;
    for (int i = B - 1; i >= 0; --i) {
      if (cur < 0) break;
      if (g2anc[i][cur] >= 0 && g2cum[i][cur] <= rem) {
	diff += 1 << i;
	rem -= g2cum[i][cur];
	cur = g2anc[i][cur];
      }
    }
    if (wei[cur] <= rem) {
      assert (g2par[cur] == -1);
      diff += 1;
    }
    cout << diff << "\n";
    last = diff;
  }
}
