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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

string f0 = "What are you doing at the end of the world? Are you busy? Will you save us?";

string fp1 = "What are you doing while sending \"";
string fp2 = "\"? Are you busy? Will you send \"";
string fp3 = "\"?";

const int N = 100100;
ll inf = 1.5e18;
ll len[N];


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> q;
  len[0] = f0.length();
  for(int i=1;i<N;++i){
    ll l = fp1.length() + fp2.length() + fp3.length() + 2 * len[i - 1];
    l = min(l, inf);
    len[i] = l;
  }
  string ans;
  REP(puella, 0, q) {
    int n;
    ll k;
    cin >> n >> k;
    k--;
    if (k >= len[n]) {
      ans += '.';
      continue;
    }
    bool ok = false;
    while (n > 0) {
      assert (k < (ll)fp1.length() + len[n - 1] + (ll)fp2.length() + len[n - 1] + (ll)fp3.length());
      if (k < (ll)fp1.length()) {
	ans += fp1[k];
	ok = true;
        break;
      }
      if (k < (ll)fp1.length() + len[n - 1]) {
	n--;
	k -= fp1.length();
	continue;
      }
      if (k < (ll)fp1.length() + len[n - 1] + (ll)fp2.length()) {
	ans += fp2[k - ((ll)fp1.length() + len[n - 1])];
	ok = true;
        break;
      }
      if (k < (ll)fp1.length() + len[n - 1] + (ll)fp2.length() + len[n - 1]) {
	k -= (ll)fp1.length() + len[n - 1] + (ll)fp2.length();
	n--;
	continue;
      }
      k -= (ll)fp1.length() + len[n - 1] + (ll)fp2.length() + len[n - 1];
      ans += fp3[k];
      ok = true;
      break;
    }
    if (not ok) {
      assert (k < f0.length());
      ans += f0[k];
    }
  }
  cout << ans << endl;
}
