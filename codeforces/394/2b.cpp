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
const ll mod = 1e9 + 7;



int main(void){
  int n;
  int l;
  cin >> n >> l;
  VI a(n), b(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
  }
  REP(i, 0, n) {
    int bias = (a[i] - b[0] + l) % l;
    set<int> t;
    set<int> s(a.begin(), a.end());
    REP(j, 0, n) {
      t.insert((b[j] + bias) % l);
    }
    if (s == t) {
      cout << "YES" << endl;
      return 0;
    }
  }
  cout <<"NO" << endl;
}
