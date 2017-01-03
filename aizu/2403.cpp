#include <iostream>
 
#include <string>
#include <map>
#include <algorithm>
#include <cassert>
#include <cstdio>
 
using namespace std;
#define N 40
#define TEST 0
 
 
    string a[N];
    int b[N],c[N];
    string d[N][N];
    int edges[N][N];
    int n;
 
typedef long long i64;
int maxv;
i64 maxi;
 
i64 edg[N];
 
void rec(i64 bits,int sum){
    if(sum<=maxv){
        return;
    }
    if((bits&1LL)==0LL)return;
    bool clique=true;
    int minind=100000;
    int mini=-1;
    for(int i=0;i<n;i++){
        if((bits&(1LL<<i))==0)continue;
        int ind=0;
	i64 is=bits&edg[i];
        for(int j=0;j<n;j++){
            if((is&(1LL<<j)))
                ind+=1;
        }
	if(bits!=(is|(1LL<<i)))
		clique=false;
        if(minind>ind){
            mini=i;
            minind=ind;
        }
    }
    if(mini==-1)assert(!"error mini==-1");
    int sub1=b[mini];
    i64 set1=1LL<<mini;
    set1|=bits&edg[mini];
    for(int i=0;i<n;i++){
        if((set1&(1LL<<i))==0)continue;
        if(i==mini)continue;
        if(set1&(1LL<<i)){
            sub1+=b[i];
        }
    }
    if(!clique){
        rec(set1,sub1);
        rec(bits&~(1LL<<mini),sum-b[mini]);
        return;
    }
    //clique
    if(maxv<sub1){
        maxv=max(maxv,sub1);
        maxi=bits;
    }
}
 
 
 
int main(void){
    int di[N][N];
    while(1){
        cin>>n;
        if(TEST)cerr<<"n="<<n<<endl;
        if(n==0)break;
        map<string,int> m;
        for(int i=0;i<n;i++){
            cin>>a[i]>>b[i]>>c[i];
            for(int j=0;j<c[i];j++){
                cin>>d[i][j];
            }
        }
        int cnt=0;
        for(int i=0;i<n;i++){
            m.insert(pair<string,int>(a[i],i));
        }
        for(int i=0;i<n;i++){
            fill_n(edges[i],n,1);
            edges[i][i]=0;
        }
        for(int i=0;i<n;i++){
            for(int j=0;j<c[i];j++){
                di[i][j]=m[d[i][j]];
                edges[i][di[i][j]]=0;
            }
	    i64 s=0;
	    for(int j=0;j<n;j++){
		s|=edges[i][j]?1LL<<j:0;
	    }
	    edg[i]=s;
        }
        if(TEST){
            for(int i=0;i<n;i++){
                for(int  j=0;j<n;j++){
                    cerr<<edges[i][j]<<" ";
                }
                cout<<":"<<b[i]<<endl;
            }
        }
        maxv=0;
        int sum=0;
        for(int i=0;i<n;i++){
            sum+=b[i];
        }
        rec((1LL<<n)-1,sum);
        cout<<maxv<<endl;
        if(TEST){
            fprintf(stderr,"set=%llx\n",maxi);
        }
    }
    return 0;
}
