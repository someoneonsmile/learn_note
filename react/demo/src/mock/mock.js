import {
  // setup,
  mock,
  Random
  // valid
} from 'mockjs';

mock('/comment/hot', {
  topComments: [],
  hasMore: true,
  'hotComments|10': [
    {
      user: {
        locationInfo: null,
        liveInfo: null,
        userId: 493560737,
        vipType: 10,
        nickname: '@cname',
        userType: 0,
        vipRights: {
          associator: null,
          musicPackage: {
            vipCode: 220,
            rights: true
          },
          redVipAnnualCount: -1
        },
        expertTags: null,
        remarkName: null,
        authStatus: 0,
        avatarUrl: () => Random.image('200x200', Random.hex(), Random.cword()),
        experts: null
      },
      beReplied: [],
      pendantData: null,
      showFloorComment: null,
      status: 0,
      commentId: 1316406400,
      content: '@cparagraph',
      time: 1543756574394,
      likedCount: '@integer(100, 10000)',
      startCount: '@integer(1000, 10000)',
      expressionUrl: null,
      commentLocationType: 0,
      parentCommentId: 0,
      decoration: null,
      repliedMark: null,
      liked: false
    }
  ],
  total: 76,
  code: 200
});
