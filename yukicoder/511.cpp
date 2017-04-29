#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int W = 320;
const int N = W * W;

ll big[W];
ll small[N];
ll ma[W];


int won[N];

int main(void){
  int n, w;
  ll h;
  cin >> n >> w >> h;
  REP(i, 0, w) {
    won[i] = -1;
  }
  REP(i, 0, n) {
    int a, x;
    ll b;
    cin >> x >> b >> a;
    a--;
    int e = a + x;
    int l = (a + W - 1) / W;
    int r = e / W;
    if (l >= r) {
      // manually
      REP(j, a, e) {
	small[j] += b;
	ma[j / W] = max(ma[j / W], small[j] + big[j / W]);
	ll res = big[j / W] + small[j];
	if (res >= h && res - b < h) {
	  won[j] = i % 2;
	}
      }
    } else {
      REP(j, a, l * W) {
	small[j] += b;
	ma[j / W] = max(ma[j / W], small[j] + big[j / W]);
	ll res = big[j / W] + small[j];
	if (res >= h && res - b < h) {
	  won[j] = i % 2;
	}
      }
      REP(j, l, r) {
	big[j] += b;
	ma[j] += b;
	if (ma[j] >= h  && ma[j] - b < h) {
	  REP(k, 0, W) {
	    int idx = j * W + k;
	    ll res = big[j] + small[idx];
	    if (res >= h && res - b < h) {
	      won[idx] = i % 2;
	    }
	  }
	}
	
      }
      REP(j, r * W, e) {
	small[j] += b;
	ma[j / W] = max(ma[j / W], small[j] + big[j / W]);
	ll res = big[j / W] + small[j];
	if (res >= h && res - b < h) {
	  won[j] = i % 2;
	}
      }
    }
  }
  int cnt = 0;
  REP(i, 0, w) {
    if (won[i] == 0) cnt += 1;
    //cout << "won[" << i << "]=" << won[i] << endl;
  }
  if (cnt * 2 == w) {
    cout << "DRAW" << endl;
  } else if (cnt * 2 < w) {
    cout << "B" << endl;
  } else {
    cout << "A" << endl;
  }
}
