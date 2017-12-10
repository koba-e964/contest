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
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  map<ll, int> freq;
  REP(i, 0, n) {
    ll a;
    cin >> a;
    freq[a]++;
  }
  VI ff;
  for (auto e: freq) {
    ff.push_back(e.second);
  }
  sort(ff.begin(), ff.end());
  int tot = 0;
  if ((int)ff.size() > k) {
    REP(i, 0, ff.size() - k) {
      tot += ff[i];
    }
  }
  cout << tot << endl;
  
}
