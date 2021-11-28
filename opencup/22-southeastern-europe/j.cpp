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
  string s;
  cin >> n >> s;
  vector<VI> acc(3, VI(3 * n + 1));
  REP(i, 0, 3 * n) {
    int ind = s[i] - 'A';
    REP(j, 0, 3) {
      acc[j][i + 1] = acc[j][i];
    }
    acc[ind][i + 1]++;
  }
  if (acc[0][3 * n] == n && acc[1][3 * n] == n && acc[2][3 * n] == n) {
    cout << "0\n";
    return 0;
  }
  REP(j, 0, 3) {
    int pos = 0;
    REP(i, 1, 3 * n + 1) {
      int x = -acc[1][3 * n] + acc[1][i] + n;
      int y = -acc[2][3 * n] + acc[2][i] + n;
      while (pos < i && acc[1][pos] <= x && acc[2][pos] <= y) {
        pos++;
      }
      if(pos > 0) pos--;
      if (acc[1][pos] == x && acc[2][pos] == y) {
        cout << 1 << endl;
        cout << pos + 1 << " " << i << " " << (char)('A' + j) << endl;
        return 0;
      }
    }
    // rotate 012 -> 120
    swap(acc[0], acc[1]);
    swap(acc[1], acc[2]);
  }
  int ind = -1;
  int pos = -1;
  REP(i, 0, 3 * n) {
    REP(j, 0, 3) {
      if (acc[j][i] == n) {
        pos = i;
        ind = j;
        goto out;
      }
    }
  }
 out:
  cout << 2 << endl;
  int oldpos = pos;
  REP(j, 0, 3) {
    if (ind != j) {
      int len = n - acc[j][oldpos];
      cout << pos + 1 << " " << pos + len << " " << (char)('A' + j) << endl;
      pos += len;
    }
  }
}
