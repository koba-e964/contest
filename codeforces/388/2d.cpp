#include <algorithm>
#include <iostream>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


// I wrote this solution after reading the editorial.
int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n), b(n);
  VI last(n, -1);
  vector<VI> bids(n);
  REP(i, 0, n) {
    cin >> a[i] >> b[i];
    a[i]--;
    last[a[i]] = b[i];
    bids[a[i]].push_back(b[i]);
  }
  vector<PI> pool;
  REP(i, 0, n) {
    if (last[i] >= 0) {
      pool.push_back(PI(last[i], i));
    }
  }
  sort(pool.rbegin(), pool.rend());
  int q;
  cin >> q;
  REP(loop_cnt, 0, q) {
    int k;
    cin >> k;
    VI l(k);
    set<int> ls;
    REP(i, 0, k) {
      cin >> l[i];
      l[i]--;
      ls.insert(l[i]);
    }
    bool found = false;
    int idx = -1;
    int snd = -1;
    REP(i, 0, pool.size()) {
      if (idx == -1 && ls.count(pool[i].second) == 0) {
	found = true;
	idx = pool[i].second;
	continue;
      }
      // Who made the second largest bid?
      if (idx >= 0 && ls.count(pool[i].second) == 0) {
        snd = pool[i].second;
	break;
      }
    }
    if (not found) {
      cout << "0 0\n";
    } else {
      cout << idx + 1;
      int bid = -1;
      // Calculate the bid [idx] made right after [snd]'s largest bid
      if (snd == -1) {
	bid = bids[idx][0];
      } else {
	// Use binary search
	int idx2 = upper_bound(bids[idx].begin(), bids[idx].end(), last[snd])
	  - bids[idx].begin();
	bid = bids[idx][idx2];
      }
      cout << " " << bid << "\n";
    }
  }
}
