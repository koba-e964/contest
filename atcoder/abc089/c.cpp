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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  map<char, ll> freq;
  string pat = "MARCH";
  REP(i, 0, n) {
    string s;
    cin >> s;
    if (find(pat.begin(), pat.end(), s[0]) != pat.end())
      freq[s[0]] += 1;
  }
  vector<pair<char, ll> > t(freq.begin(), freq.end());
  int m = t.size();
  ll tot = 0;
  REP(i, 0, m) {
    REP(j, i + 1, m) {
      REP(k, j + 1, m) {
	tot += t[i].second * t[j].second * t[k].second;
      }
    }
  }
  cout << tot << endl;
}
