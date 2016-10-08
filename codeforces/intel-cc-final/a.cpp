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
  string s, t;
  cin >> s >> t;
  map<string, int> tbl;
  tbl.insert(pair<string, int>("sunday", 0));
  tbl.insert(pair<string, int>("monday", 1));
  tbl.insert(pair<string, int>("tuesday", 2));
  tbl.insert(pair<string, int>("wednesday", 3));
  tbl.insert(pair<string, int>("thursday", 4));
  tbl.insert(pair<string, int>("friday", 5));
  tbl.insert(pair<string, int>("saturday", 6));
  int ss = tbl[s];
  int ts = tbl[t];
  ts = (ts - ss + 7) % 7;
  cout << (ts == 0 || ts == 2 || ts == 3 ? "YES" : "NO") << endl;
}
