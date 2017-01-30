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

VI answer(vector<VI> &res) {
  vector<VI> freq(50, VI(4, 0));
  REP(j, 0, res.size()) {
    REP(k, 0, 50) {
      freq[k][res[j][k]]++;
    }
  }
  VI ans(50);
  REP(k, 0, 50) {
    int mini = 0;
    REP(j, 1, 4) {
      if (freq[k][j] > freq[k][mini]) {
	mini =j;
      }
    }
    ans[k] = mini;
  }
  return ans;
}

int main(void){
  REP(i, 0, 200) {
    vector<VI> res(30, VI(50));
    REP(j, 0, 30) {
      REP(k, 0, 50) {
	cin >> res[j][k];
      }
    }
    VI ans = answer(res);
    vector<VI> res2;
    vector<PI> srt;
    REP(i, 0, 30) {
      int cor = 0;
      REP(k, 0, 50) {
	cor += res[i][k] == ans[k];
      }
      srt.push_back(PI(cor, i));
    }
    sort(srt.rbegin(), srt.rend());
    REP(j, 0, 10) {
      res2.push_back(res[srt[j].second]);
    }
    VI ans2 = answer(res2);
    REP(k, 0, 50) {
      cout << ans2[k] << (k == 49 ? "\n" : " ");
    }
  }
}
