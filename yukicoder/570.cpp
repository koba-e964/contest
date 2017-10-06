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
  vector<pair<int, char> > pool;
  REP(i, 0, 3) {
    int h;
    cin >> h;
    pool.push_back(make_pair(-h, 'A' + i));
  }
  sort(pool.begin(), pool.end());
  REP(i, 0, 3) {
    cout << pool[i].second << endl;
  }
}
